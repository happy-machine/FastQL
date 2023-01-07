import os
import random
import torch
from fastqlapi import fastql_server
from diffusers import StableDiffusionPipeline, EulerDiscreteScheduler

model_id = os.getenv('MODEL_ID', 'stabilityai/stable-diffusion-2')
scheduler = EulerDiscreteScheduler.from_pretrained(model_id, subfolder='scheduler')
pipe = StableDiffusionPipeline.from_pretrained(model_id, scheduler=scheduler, torch_dtype=torch.float16)
pipe = pipe.to("cuda")

def infer(**kwargs):
  seed = kwargs.get('seed', torch.random.seed())
  images = pipe(
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

fastql_server.start(callback=infer, query_name="Model", 
  args={
    "prompt": {
      "type": "String",
      "description": "A description of the image you want to generate, try to be specific ie.Cute Grey Cat with blue eyes, wearing a bowtie"
    },
    "number_of_images": {
      "type": "Int",
      "description": "The number of images to generate"
    },
    "seed": {
      "type": "Int",
      "description": "Set a seed for deterministic output"
    },
    "guidance_scale": {
      "type": "Float",
      "description": "A parameter that controls how much the image generation process follows the text prompt"
    },
    "number_inference_steps": {
      "type": "Float",
      "des  cription": "The number of inference steps to run, the higher the number the more detailed the image will be"
    }
  }, 
  fields={
    "images": {
      "type": "[String]",
      "Description": "A list of urls to the generated images"
    },
    "seed": {
      "type": "Int",
      "Description": "The seed used to generate the images"
    },
 })