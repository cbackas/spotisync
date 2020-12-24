FROM python:3-alpine

RUN apk add tzdata

ENV TZ='America/Chicago'

ADD ./spotisync spotisync/
RUN pip install spotipy

# env variables for spotipy to work
ENV SPOTIPY_CLIENT_ID=
ENV SPOTIPY_CLIENT_SECRET=
ENV SPOTIPY_REDIRECT_URI=

# config folder is where token cache lives
WORKDIR /
RUN mkdir config

WORKDIR /spotisync
CMD [ "python", "-u", "core.py" ]