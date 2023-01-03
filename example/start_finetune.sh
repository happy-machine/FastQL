cd ~ && git clone https://github.com/huggingface/diffusers
cd diffusers && pip install -e .
cd ~/diffusers/examples/dreambooth
export PATH="/home/ubuntu/.local/bin:$PATH"
pip install -r requirements.txt
pip install markupsafe==2.0.1 datasets
# TBD ^ what up?
mkdir uploads saved_models images
chmod 777 uploads saved_models images
mv /model/* ./ && mv /model/.env ./
chmod 777 finetune.sh start_finetune.sh
source .env
accelerate config default
ruby -run -e httpd ./images -p 8080 &
python3 finetune_and_inference.py