# Description
This bash script helps install .AppImage files on your session.
You should have at the end the desktop icon in your applications.

# Dependencies
.AppImage format may require FUSE to work properly.<br>
You can check if it is installed with this command:<br>
```sh
which fusermount
```

If nothing returns, consider to install it.<br>
```sh
sudo apt update
sudo apt install fuse3
```

# Quick Start
1. **Clone the repository**
```sh
git clone https://github.com/SwaonD/appimage_installer.git
```
2. **Install the .AppImage**
```sh
./appimage_installer/main.sh <my_file>.AppImage
```

# Script Installation
To be able to use the script with the command `appimage-install`.
1. **Run the installer**
```sh
./appimage_installer/install.sh
```
2. **Add the binary directory to the PATH**
```sh
echo "PATH=$PATH:~/.local/bin" >> ~/.profile
```
3. **Reset profile**
```sh
source ~/.profile
```

# Usage
```sh
appimage-install <my_file>.AppImage
```

# Details
Every variables are in the src/variables.sh file, including the directory path used to store the applications (APP_DIR).
