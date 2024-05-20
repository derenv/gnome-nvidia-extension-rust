#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2024 Deren Vural
# SPDX-License-Identifier: GPL-3.0-or-later

#####
# Name:
# install_schemas.sh
#
# Description:
# Installs gtk schema to system
#
# Made:
# 13/10/2022
#
# Made by:
# Deren Vural
#
# Notes:
#
#####

# Variables
SCHEMA_DIR="$HOME/.local/share/glib-2.0/schemas"
SCHEMA_FILE="com.gtk_d.NvidiaMonitorRust.gschema.xml"

# Create schema directory if it doesn't exist
if [[ -d $SCHEMA_DIR ]]; then
	echo "$SCHEMA_DIR already exists.."
else
	echo "$SCHEMA_DIR needs created.."
	mkdir -p "$SCHEMA_DIR"
fi

# Copy schema file across
if [[ -f "$SCHEMA_DIR/$SCHEMA_FILE" ]]; then
	echo "$SCHEMA_FILE already exists, overwriting.."
	cp "./src/$SCHEMA_FILE" "$SCHEMA_DIR/"
else
	echo "$SCHEMA_FILE needs copied to $SCHEMA_DIR.."
	cp "./src/$SCHEMA_FILE" "$SCHEMA_DIR/"
fi

# Compile schemas
echo "Compiling schemas.."
glib-compile-schemas "$SCHEMA_DIR/"
