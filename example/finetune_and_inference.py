import os
import random
import torch
from fastqlapi import fastql_server
from lib import model

def run(**kwargs):
    if (kwargs.get('fine_tune_photo_description', None) is not None):
      os.environ["INSTANCE_PROMPT"] = kwargs['fine_tune_photo_description']
      return model.finetune(**kwargs)

    elif (kwargs.get('prompt', None) is not None):
      seed = kwargs.get('seed', random.getrandbits(32))
      images = model.pipe(
        [kwargs['prompt']] * kwargs.get('number_of_images', 1),
        guidance_scale=kwargs.get('guidance_scale', 7.5),
        generator=torch.Generator("cuda").manual_seed(seed),
        num_inference_steps=kwargs.get('number_inference_steps', 15)
      ).images
      messages = []
      for image in images:
        hash = random.getrandbits(64)
        image.save(f"images/{hash}.png")
        messages.append(f"http://{os.environ['PUBLIC_IP']}:{os.environ['SERVER_PORT']}/{hash}.png")
      return {
        'images': messages,
        'seed': seed
      }

fastql_server.start(callback=run, query_name="Model",
  args={
    "prompt": {
      "type": "String",
      "description": "A description of the image you want to generate, try to be specific ie.Cute Grey Cat with blue eyes, wearing a bowtie"
    },
    "fine_tune_photo_description": {
      "type": "String",
      "description": "ie. a photo of a dog, you will need to provide either a gdrive folder link to gdrive_folder_of_images_link or an array of gdrive ids to gdrive_image_link_ids, it could take about three minutes to fine tune the model"
    },
    "gdrive_folder_of_images_link": {
      "type": "String",
      "description": "ie. https://drive.google.com/drive/folders/17g_m3eaBA6SJQP-xppGkAHXYoKCLObZg"
    },
    "gdrive_image_link_ids": {
      "type": "[String]",
      "description": "ie. [\"17g_m3eaBA6SJQP-xppGkAHXYoKCLObZg\", \"t9a_m3eaBAweJQP-xppGkAHXYoKCLObZg\"]"
    },
    "number_of_images": {
      "type": "Int",
    },
    "seed": {
      "type": "Int",
      "description": "Set a seed for deterministic output"
    },
    "guidance_scale": {
      "type": "Float",
    },
    "number_inference_steps": {
      "type": "Float",
    }
  },
  fields={
    "images": {
      "type": "[String]"
    },
    "seed": {
      "type": "Int"
    },
 })