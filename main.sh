#!/bin/bash

CONFIG_DIR="~/.config/gpgpass"
EXPANDED_DIR="`eval echo ${CONFIG_DIR//>}`"

init () {
    if [ -d $EXPANDED_DIR ]; then
        echo "gpgpass config dir detected!
            
        !!!!Please make sure that you have backed-up your secret keys before proceeding!!!!
        "
        while true; do
            read -p "Would you like to delete older keys and re-initiate gpgpass? [Y/n] " yn
            case $yn in
                [Yy]* ) rm -rf "`eval echo ${EXPANDED_DIR//>}`" && echo "Successfully deleted $EXPANDED_DIR"
                break
                ;;
                [Nn]* ) exit 0;;
                * ) echo "Please answer yes or no.";;
            esac
        done
    fi
    
    echo "Git url of the repo that you wish to use with gpgpass: "
    read repo_url
    
    # Validate git url
    git ls-remote $repo_url > /dev/null 2>&1
    if [ $? != 0 ];then
        echo "Invalid git url!"
        exit 0
    fi

    # Clone repo with existing credentials
    mkdir -p $CONFIG_DIR
    git clone $repo_url $CONFIG_DIR
}

if [ "$1" == "init" ]; then
    init
fi