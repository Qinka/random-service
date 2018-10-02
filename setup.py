import  setuptools

with open("README.md", "r") as f:
    long_description = f.read()

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
#        'wsgi',
        ]
    )
