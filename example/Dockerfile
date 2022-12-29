FROM pytorch/pytorch:1.12.1-cuda11.3-cudnn8-runtime
RUN apt-get update && apt-get -y install ruby python3-pip
RUN pip install --upgrade diffusers transformers accelerate scipy safetensors accelerate fastqlapi==0.3.3 
RUN pip install --upgrade --no-cache-dir gdown
RUN mkdir model
COPY . /model
WORKDIR /model
RUN mkdir images
CMD bash start_infer.sh
