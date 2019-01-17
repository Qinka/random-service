import setuptools
import os

with open("README.md", "r") as f:
    long_description = f.read()

def find_files(path):
    return map(lambda x:os.path.join(path,x), os.listdir(path))

setuptools.setup(
    name = "random-service",
    version = "0.1.0",
    description = "Random services written in Python",
    long_description = long_description,
    long_description_content_type = "text/markdown",
    url = "https://github.com/Qinka/random-service",
    author = "Johann Lee",
    author_email = "qinka@live.com",
    license = "GPLv3",
    packages = setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: GPLv3 License",
        "Operating System :: OS Independent",
    ],
    install_requires=[
        'flask',
        ],
    data_files = [
        ('', find_files('random_service/static')),
        ('', find_files('random_service/templates'))
        ]
    )

