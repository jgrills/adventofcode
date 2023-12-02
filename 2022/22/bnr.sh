#! /bin/bash

# Make sure we run the first time
newer=1
touch ./.timestamp

while true; do

    # Check all the files in the directory for an update
    for filename in *; do
      [[ ${filename} -nt .timestamp ]] && newer=1
    done

    # We want to be responsive, but don't want to burn a CPU
    # and don't care enough to make this properly  inotify based.
    #
    # Sleeping here also helps to ensure the files are done being
    # changed before building.
    sleep 0.05

    # Do the build and run it if the build was successful
    if (( newer )); then
      clear
      ./b.sh && ./r.sh

      # mark ourselves up to date and update the hidden timestamp file
      newer=0
      touch ./.timestamp
    fi
done
