git clone https://github.com/huggingface/diffusers
cd diffusers
pip install -e .
cd ~/diffusers/examples/dreambooth
sudo apt-get update && sudo apt-get install -y python3-pip
pip install -r requirements.txt
pip install markupsafe==2.0.1
cd ~/diffusers/examples/dreambooth
sudo mkdir uploads saved_models images
sudo chmod 777 uploads saved_models images
sudo mv ~/start_finetune.sh ./start_finetune.sh 
sudo mv ~/finetune_and_infer.py ./finetune_and_infer.py 
sudo mv ~/finetune.sh ./finetune.sh
sudo mv ~/.env ./.env
source .env
export PATH="/home/ubuntu/.local/bin:{$PATH}"
accelerate config default
ruby -run -e httpd ./images -p 8080 &
python3 finetune_and_infer.py