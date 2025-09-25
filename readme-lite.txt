Thank you for downloading Ultimate S Lite!


-Instructions-
Copy the atmosphere and ultimate folders to the root of your SD card


If you haven't set up smash ultimate modding as a whole, here's some wonderful guides!
Setting up CFW on switch - https://switch.homebrew.guide/
Modding Smash Ultimate on switch - https://gamebanana.com/tuts/12827
Modding Smash Ultimate on yuzu - https://youtu.be/uKKQl7QFBMA


-Dependecies-
Dependencies should come with the download, but if for some reason they aren't there, or your game is crashing, try updating these.

libparam_config.nro - https://github.com/CSharpM7/lib_paramconfig
libcsk_collection.nro - https://gamebanana.com/mods/499008 (just grab the nro file from this. Do NOT grab the files in ultimate/mods from this download)
libstage_config.nro - https://github.com/ThatNintendoNerd/stage_config
libarcropolis.nro - https://github.com/Raytwo/ARCropolis
libnro_hook.nro - https://github.com/ultimate-research/nro-hook-plugin
libsmashline_plugin.nro - https://github.com/HDR-Development/smashline/releases/latest
skyline - https://github.com/skyline-dev/skyline
libone_slot_eff.nro - https://github.com/blu-dev/one-slot-effects/releases

Place the plugins in "/atmosphere/contents/01006A800016E000/romfs/skyline/plugins"
Place skyline in "/atmosphere/contents/01006A800016E000"


-Common Issues-
Q: "My game is crashing! What do I do?"
A: There could be many things that could be causing this! Try going down this list to make sure that it isn't a user error:
1. Make sure your version of smash ultimate is update 13.0.3 (other updates do not work)
2. Make sure training modpack is not enabled (this means no training modpack plugin, and no param hook plugin)
3. Make sure your plugin folder is CLEAN (this means the only plugins in the folder are libparam_config, libcsk_collection, libstage_config, libarcropolis, libnro_hook, libsmashline_plugin and libarena_latency_slider).

Q: "The character's attacks are messed up! They're doing their vanilla animations but the custom attack effects!"
A: This sounds like you have a mod conflict. If you're on switch, this is easy because arcropolis will tell you this, and create a conflicts.json for you (which you can use to disable the conflicting mods).
If you're on Emulator, this is a more involved process. If you really just want Ultimate S to work, empty out your ultimate/mods folder and ensure its just "Ultimate S Arcropolis" and "Ultimate S Stages" there.
Check that this boots up and works just fine. If it does, it's definitely a mod conflict. Slowly add in your skin mods until you find where the conflict is, its usually a conflict where the skin mod edits a character's 
motion folder (example: https://gamebanana.com/wips/75639) , or a projectile's model (example: https://gamebanana.com/mods/350254). You'll have to delete the folders which conflict with Ultimate S or disable those mods.

Q: "The character's attacks are messed up! They're doing their custom animations but the vanilla attack effects!"
A: This also sounds like a mod conflict! Refer to above.

Q: "I'm trying to access Ultimate S Settings and custom gamemodes and I'm not getting the menu!"
A: You're likely on emulator, which doesn't support web menus. 
Pressing Ultimate S Settings will toggle both Ultimate S Mechanics and Shorthop Aerial Macro as a workaround.
If you want to remove one of these you can go to "ultimate/ult-s/sys-flags/" and remove either "mecahnics.flag" or "sh.flag"

Pressing custom gamemodes will let you play with Airdash Mode, Parry Mode, Hitfall Mode and Fighter Mode by default. 
If you want to change this, go to "ultimate/ult-s/gamemode-default.txt" and add the modes you want to play:
"turbo" - Turbo Mode
"vampire" - Vampire Mode 
"fgmode" - Fighter Mode 
"sixtyfour" - Smash 64 Mode 
"superboss" - Superboss Mode 
"airdash" - Airdash Mode
"angles" - Random Angles Mode
"hitfall" - Hitfall Mode 
"itemduel" - Item Duel Mode 
"hitstun" - Hitstun Mode 
"effects" - Random Effects Mode 
"parry" - Parry Mode

Q: "The game is going too fast!"
A: You're likely on Yuzu or one of its forks. Make sure game speed is set to 50%, by going to emulation -> config -> system -> limit speed percent (on yuzu) and setting it to 50%, and this should make your game run at regular speeds.
If you're on Ryujinx, try download RTSS Rivatuner here: https://www.guru3d.com/download/rtss-rivatuner-statistics-server-download
and follow these instructions: https://www.guru3d.com/page/how-to-use-rivatuner-to-limit-fps

Q: "The game is just vanilla!"
A: Try going through these steps:
1. Check your title screen, does it say Arcropolis in the top right? If not, arcropolis isn't running, make sure you have skyline AND arcropolis installed correctly.
2. Check your title screen, what version is it? Make sure it is 13.0.3, and no other version.
3. Make sure that Ultimate S is enabled (either by mod manager on the switch, or inside of the mods folder on Yuzu).
4. If you are on emulator, make sure that there is a file called "legacy_discovery" at ultimate\arcropolis\config\2470593114292646594\15904440331188662786
If there isn't, you can make one by creating a file (no extension on the end) with "True" written in it



Any other issues or queries should be directed to the discord server (https://discord.gg/YXaPzC5WGd)

Have fun!



