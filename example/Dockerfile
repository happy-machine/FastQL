FROM pytorch/pytorch:1.13.1-cuda11.6-cudnn8-runtime
RUN apt-get update && apt-get -y install ruby python3-pip git
RUN pip install --upgrade diffusers transformers accelerate scipy safetensors fastqlapi==0.3.5
RUN pip install --upgrade --no-cache-dir gdown
COPY . /model
WORKDIR /model
RUN mkdir images && chmod 777 images
CMD bash start_inference.sh
