import os
import random
import torch
import shutil
from fastqlapi import fastql_server
from subprocess import Popen
from diffusers import StableDiffusionPipeline, EulerDiscreteScheduler
import gdown

model_id = os.getenv('MODEL_ID', 'stabilityai/stable-diffusion-2')
pipe = StableDiffusionPipeline.from_pretrained(model_id, torch_dtype=torch.float16)
pipe = pipe.to("cuda")

def download_images(**kwargs):
  '''
  We have no mutation option at the moment
  the below query should really be a mutation but this is just
  an example
  '''
  output = "./uploads"
  shutil.rmtree(output, ignore_errors=True)
  os.mkdir(output)
  if (kwargs.get('gdrive_image_link_ids', None) is not None):
    for id in kwargs.get('gdrive_image_link_ids'):
      url = f'https://drive.google.com/uc?id={id}'
      gdown.download(url, output, quiet=False)
  else:
    if (kwargs.get('gdrive_folder_of_images_link', None) is not None):
      folder = kwargs.get('gdrive_folder_of_images_link')
      gdown.download_folder(folder, quiet=True, output=output)
  Popen(["bash" ,"/home/ubuntu/diffusers/examples/dreambooth/finetune.sh"])
  pipe = StableDiffusionPipeline.from_pretrained("./saved_models", torch_dtype=torch.float16)
  pipe = pipe.to("cuda")
  return {
    'images': ['model finetuned successfully'],
    'seed': 0
  }


def run(**kwargs):
    if (kwargs.get('fine_tune_photo_description', None) is not None):
      print('desc', kwargs['fine_tune_photo_description'])
      os.environ["INSTANCE_PROMPT"] = kwargs['fine_tune_photo_description']
      return download_images(**kwargs)

    elif (kwargs.get('prompt', None) is not None):
      seed = kwargs.get('seed', torch.random.initial_seed())
      images = pipe(
        [kwargs['prompt']] * kwargs.get('number_of_images', 1),
        guidance_scale=kwargs.get('guidance_scale', 7.5),
        generator=torch.Generator("cuda").manual_seed(seed),
        num_inference_steps=kwargs.get('number_inference_steps', 15)
      ).images
      messages = []
      for image in images:
        hash = random.getrandbits(128)
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
      "description": "Your sexy prompt"
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