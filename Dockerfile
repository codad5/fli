FROM rust:1.81

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Set the default user
ENV USER=codad5


