import os
import subprocess
import gdown
import shutil
import torch
from diffusers import StableDiffusionPipeline

class Model:
  def __init__(self):
    self.pipe = None
    self.model_id = os.getenv('MODEL_ID', 'stabilityai/stable-diffusion-2')
    self.pipe = StableDiffusionPipeline.from_pretrained(self.model_id, torch_dtype=torch.float16)
    self.pipe = self.pipe.to("cuda")
  def pipe(self, new_pipe):
    self.pipe = self.pipe(new_pipe)
  def finetune(self, **kwargs):
    self.pipe = None
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
    try:
      subprocess.call("./finetune.sh")
    except subprocess.CalledProcessError as e:
      print(e.output)
      quit()
    self.pipe = StableDiffusionPipeline.from_pretrained("./saved_models", torch_dtype=torch.float16)
    self.pipe = self.pipe.to("cuda")
    return {
      'images': ['model finetuned successfully'],
      'seed': 0
    }

model = Model()