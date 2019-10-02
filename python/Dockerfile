FROM python:3.7-alpine
LABEL maintainer="Kyle Galloway <kyle.galloway@cfdrc.com>"

# Install python packages
COPY requirements.txt /
RUN pip install -r requirements.txt

# Expose port to allow the host to connect to the server inside the container
EXPOSE 8000

# Create source directory
ENV SRC_DIR /src
RUN mkdir ${SRC_DIR}
WORKDIR ${SRC_DIR}
VOLUME ${SRC_DIR}

# CMD ["/bin/sh"]
CMD ["python", "mysite/manage.py", "runserver", "0:8000"]

