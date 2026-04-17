# SN-Modding-Icon-Factory
Repository for storing, organizing, and adding icons for the Subnautica Modding discord server


## Adding your own
You're welcome to add your own icons and events! You can either create a pull request if you're comfortable with that, or send us the icon and we can add it for you. The bot can automatically merge together various images based on a layering system, the layers being determined by the number at the end of the file name. For example `1.png` would be the first layer, but so would `testExample_7_layer_test_1.png`. 

There are four default layers, that make up the background water, the subnautica logo outline, the main S of the subnautica logo, and the animated gears on top. These are automatically applied if the event in question doesn't have any images or gifs supplied for that layer. And yes, this means the gifs are given transparency. Any black pixels (within a small color margin) in gifs are treated as transparent and are cleared for the lower layers to pass through. 

If you don't need any of the layers to be merged or applied, and you simply have a finished image, you can add that directly under the name `finalImage.png` (.gif also accepted). The bot will skip any merging steps for any events that already have a `finalImage` supplied. 

If you need more help, you're free to ask for it or read through the [Icon Help Expanded](https://github.com/EldritchCarMaker/SN-Modding-Icon-Factory/edit/main/README.md#icon-help-expanded) section

Events are handled by an annoyingly complicated series of interpreters, that try to parse most possible human readable dates and time frames. Any condition examples in the `testConditions` folder are fully working. The rough list is
- Arbitrary date range (eg, dec 2nd-jan 1st
- Days of the week (eg, second monday of january)
- simple month-day dates (eg, Jan 1st)
- monthly events (eg, june)

One noted range that is not currently implemented is "week of" ranges. Such as "The week of february 17th" or "the week after october 12th".


The final file required, after the icon and the condition, is a json file named `EventInfo.json`. This file contains the information that would be provided to a person using the `/current-event` command in discord. This includes the name of the event, a description of it (as short or long as you'd like), and optionally a color, an author name (likely who made the icon, but doesn't have to be), and space for a footer.


## Icon Help Expanded

[Resources folder](https://github.com/EldritchCarMaker/SN-Modding-Icon-Factory/tree/main/Resources) has a bunch of icons for help getting started, and Aci uploaded a [small video for color shifting icons to match pride flags](https://youtu.be/89zODfgY5Fo)
