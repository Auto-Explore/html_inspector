# html/semantics/embedded-content/media-elements/ready-states/autoplay-with-slow-text-tracks.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/ready-states/autoplay-with-slow-text-tracks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>autoplay with slow text tracks</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>
setup({ single_test: true });
// https://html.spec.whatwg.org/#ready-states says:
//
// HAVE_FUTURE_DATA: "the text tracks are ready".
// HAVE_ENOUGH_DATA: All the conditions described for the HAVE_FUTURE_DATA state are met, ...
//
// When the ready state of a media element whose networkState is not NETWORK_EMPTY changes,
// the user agent must follow the steps given below:
// If the new ready state is HAVE_ENOUGH_DATA
// (autoplay)
//
// So if the text tracks are not yet ready, we can't autoplay.

var started = 0;
var numOfTests = 5;

function createTest() {
  var video = document.createElement('video');
  video.src = getVideoURI('/media/movie_5');
  video.autoplay = true;
  video.muted = true;
  video.controls = true;
  video.onplaying = function() {
    started++;
    assert_equals(track.track.cues.length, 1);
    if (started === numOfTests) {
      done();
    }
  };
  var track = document.createElement('track');
  track.src = '/media/foo.vtt?pipe=trickle(d2)';
  track.default = true;
  video.appendChild(track);
  document.body.appendChild(video);
}
for (var i = 0; i < numOfTests; ++i) {
  createTest();
}
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
  "source_name": "html/semantics/embedded-content/media-elements/ready-states/autoplay-with-slow-text-tracks.html"
}
```
