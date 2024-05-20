<!--
SPDX-FileCopyrightText: 2024 Deren Vural
SPDX-License-Identifier: GPL-3.0-or-later
-->

# Roadmap

- [x] Basic Functionality
- [ ] Match GWE Functionality
	- [ ] Full statistics
	- [ ] Add fan-curve support
	- [ ] Add clock speed support
- [ ] Improve on GWE Functionality
	- [ ] Customizable tabs/views of statistics
        - [x] Can set name of each tab/view
        - [x] Can set the list of properties in each tab/view
        - [x] Can re-order list of properties in each tab/view
        - [ ] Can set icon of each tab/view
    - [ ] Better overclocking support
- [ ] Improve settings application support
	- [x] Add ability to open settings app
	- [ ] Add ability to set settings application in the app settings
- [ ] Add provider application support
	- [x] Add ability to set provider application in the app settings
	- [x] nvidia-settings (Nvidia)
	- [x] nvidia-smi (Nvidia)
	- [x] optimus (Nvidia)
		- [ ] Cover bug described [here](https://github.com/ethanwharris/gnome-nvidia-extension/issues/206)
	- [ ] Find suitable AMD-based app

## Basic Functionality
match old extension, basic design of app

## Full Statistics
match GWE

## Add fan-curve support
match GWE

## Add Clock speed support
match GWE

## Customizable Tabs of Statistics
i.e. improve on GWE

## Improve settings app support
Can open settings app (nvidia-settings), this should be set within the app settings.

## Improve provider app support
### Nvidia
Currently, the app allows interacting with GPU management interfaces like Nvidia's [nvidia-settings](https://github.com/NVIDIA/nvidia-settings) and [nvidia-smi](https://developer.nvidia.com/nvidia-system-management-interface), and [optimus](https://www.nvidia.com/en-gb/geforce/technologies/optimus/).

#NOTE There is an issue in the [original extension](https://github.com/ethanwharris/gnome-nvidia-extension) with nvidia-settings and nvidia-smi (and thus also optimus) where fan speed is unavailable to query and thus breaks the functionality of the extension. This highlights an issue - what to do if results given by the provider are invalid? How can we check this when modifying GPU views so that the user cannot add invalid statistics? Should the statistic instead show an error message (i.e. "This statistic is not valid for this GPU..")?

### AMD
Hopfully we can support some equivalent provider for AMD, for example [radeon-profile](https://github.com/marazmista/radeon-profile), a GPU management interface developed by [marazmista](https://github.com/marazmista) (though it has not been updated since 2020...).

#NOTE This will require a name change!

### Mesa/Nouveau
Currently no research has been done re the open source drivers, as to my knowledge there are no equivalent GPU management interfaces for these.
