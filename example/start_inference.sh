ruby -run -e httpd ./images -p 8080 &
source .env
python3 inference_only.py