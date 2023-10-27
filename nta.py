#!/usr/bin/env python3

# Standard imports
import shutil
import subprocess
import os
import urllib.request

# Local imports
import share
import folders
import regex
from ntrkservices import Services

# Create logger
logger = share.createlogger('ntaupgrade')

class NtaUpgrade():
    # Status Constants
    STATUS_ENABLED = 1
    STATUS_DISABLED = 2
    STATUS_PAUSED = 4
    STATUS_INCOGNITO = 8
    STATUS_ERROR = 128

    def __init__(self):
        # Set folder locations
        self._repo = 'https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/'
        self._git_download = 'https://github.com/l1nux-th1ngz/nta/archive/master.zip'

        self._webowner = share.get_owner(folders.webdir)

        self.location = share.find_nta()  # Where has Nta been installed?

        self.username = share.find_username(self.location)

        if self.location == '':
            logger.error('Unable to find Nta location')
        else:
            logger.info(f'Nta Location: {self.location}')

        logger.info(f'Nta Username: {self.username}')

        self.__find_latest_version()

    def __find_latest_version(self):
        """
        Get the latest version from bl_nta.txt
        """
        tempbl = f'{folders.tempdir}/bl_nta.txt'
        filelines = list()

        filelines = share.load_file(tempbl)

        if len(filelines) == 0:
            logger.warning(f'{tempbl} is missing. This could be due to a recent reboot, or Nta block list is not enabled')
            return

        for line in filelines:
            # Use regex to find the correct line
            matches = regex.NtaVersion.match(line)
            if matches is not None:
                self._latestversion = matches[1]
                logger.info(f'Latest version of Nta is: {self._latestversion}')
                break

    def __notification_create(self):
        """
        Create latestversion.php with the necessary PHP code
        """
        latestversionphp = f'{folders.webconfigdir}/latestversion.php'

        with open(latestversionphp, 'w') as f:
            f.write('<?php\n')
            f.write(f"$upgradenotifier->latestversion = '{self._latestversion}';\n")
            f.write('?>\n')

        share.set_owner(latestversionphp, self._webowner.st_uid, self._webowner.st_gid, 0o664)

    def __notification_delete(self):
        """
        Delete latestversion.php
        """
        logger.info('Deleting upgrade notification')
        share.delete(f'{folders.webconfigdir}/latestversion.php')

    def __download_latest_version(self):
        """
        Download the latest version from the provided URL
        """
        logger.info('Downloading the latest version of Nta...')
        try:
            latest_version_url = self._repo + 'nta.py'
            response = urllib.request.urlopen(latest_version_url)
            latest_version_content = response.read()
            latest_version_path = os.path.join(self.location, 'nta.py')

            with open(latest_version_path, 'wb') as latest_version_file:
                latest_version_file.write(latest_version_content)

        except Exception as e:
            logger.error(f'Failed to download the latest version: {str(e)}')

    # The rest of the class methods remain the same

def main():
    logger.setLevel(20)  # Change log level to INFO
    share.logger.setLevel(20)

    print('Nta Upgrader')
    services = Services()
    ntaupgrade = NtaUpgrade()

    share.userutils.check_root()
    ntaupgrade.is_upgrade_available()
    ntaupgrade.do_upgrade()
    ntaupgrade.delete_old_files()

    print('Restarting Nta...')
    services.restart_nta()

    print('Nta upgrade complete :-)')

if __name__ == "__main__":
    main()
