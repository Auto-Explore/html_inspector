# html/semantics/embedded-content/media-elements/track/track-element/no-cuechange-before-play.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/no-cuechange-before-play.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Ensure that the 'cuechange' event is not fired before video playback has begun.</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
promise_test(function(t) {
    let video = document.createElement('video');
    video.src = getVideoURI('/media/movie_5');
    video.preload = 'auto';

    // Create a track element. The 'cuechange' event should not be fired.
    let track = document.createElement('track');
    track.oncuechange = t.unreached_func('The \`cuechange\` event should not be fired');

    let videoWatcher = new EventWatcher(t, video, 'canplaythrough');
    let trackWatcher = new EventWatcher(t, track, ['cuechange', 'load'])

    track.src = 'resources/captions-fast.vtt';
    track.kind = 'captions';
    track.default = true;
    track.track.mode = 'showing';
    video.appendChild(track);

    return Promise.all([videoWatcher.wait_for('canplaythrough'), trackWatcher.wait_for('load')]);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/no-cuechange-before-play.html"
}
```
