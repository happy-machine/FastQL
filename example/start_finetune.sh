cd ~
git clone https://github.com/huggingface/diffusers
cd diffusers
pip install -e .
cd ~/diffusers/examples/dreambooth
pip install -r requirements.txt
pip install markupsafe==2.0.1
# TBD ^ what up?
cd ~/diffusers/examples/dreambooth
sudo mkdir uploads saved_models images
sudo chmod 777 uploads saved_models images
sudo mv ~/example/* ./
source .env
export PATH="/home/ubuntu/.local/bin:{$PATH}"
accelerate config default
ruby -run -e httpd ./images -p 8080 &
python3 finetune_and_inference.py