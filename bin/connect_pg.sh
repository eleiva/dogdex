# docker run \
# --name catdex-db \
#  -e POSTGRES_PASSWORD=mypassword \
#  -p 5432:5432 \
#  -d \
#  postgres:12.3-alpine

# sudo systemctl stop docker.socket
# sudo systemctl start docker.service
# systemctl status docker.service
# sudo docker ps -a
# sudo docker start dogdex-db
export PGPASSWORD='mypassword'
psql -h localhost -p 5432 --username=postgres
