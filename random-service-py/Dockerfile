FROM python:3
LABEL maintainer="Johann Lee <me@qinka.pro>"

ADD . /application
RUN pip install flask
EXPOSE 3000
ENV FLASK_APP=/application/random_service
CMD flask run --host=0.0.0.0 --port=3000
