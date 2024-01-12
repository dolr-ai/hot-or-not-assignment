# Brief

Build a video feed using your choice of frontend framework:

- Fetch video list from API.
- Display videos in a swipe-able feed (like reels/shorts)
- Videos on swipe should pause the video before, play the next video.
- Implement virtual-list to limit the creation of DOM elements (optional).

# Submittable

- URL to the source code repository
- URL to the deployed app

# Todos:
- Single tap to pause/play a video
- Mute button should mute/unmute the all videos
- On scroll the video going out of view should pause and the incoming video should play automatically.
- Other overlay items/buttons should not do anything.
- The videos should keep playing in a loop.
- The video should restart (play from the begenning) if it is visited again.
- The feed should be infinitely scrollable. There are only 40 video links in the mentioned linked. However, you can start looping the videos.

# Additional Information

- You can hit the endpoint to fetch the list of videos:
https://65803cdd6ae0629a3f54b7fb.mockapi.io/api/videos/mp4

# Design

- [Here’s the link to the design file](https://www.figma.com/file/QUj98myo5CWqopv2Kn4LC6/Untitled?node-id=0%3A1&t=Owr1U6DhNwVRE1nQ-1)

Important: The figma design file contains only one screen and some behaviours are not explicitly defined, which may cause confusion.
Please visit https://hotornot.wtf and use it to understand the expected outcome. The assignment will be judged firstly on the basis of functionality (Your implementation should work at least on par or better than the feed in the website) and secondly on UI polish (This is a bonus)
