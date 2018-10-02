FROM python:3
MAINTAINER Johann Lee <me@qinka.pro>

ADD . /application
RUN pip install /application
EXPOSE 3000
CMD python random_service
