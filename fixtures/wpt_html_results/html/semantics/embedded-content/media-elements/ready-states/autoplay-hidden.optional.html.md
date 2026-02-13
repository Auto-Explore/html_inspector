# html/semantics/embedded-content/media-elements/ready-states/autoplay-hidden.optional.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/ready-states/autoplay-hidden.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>autoplay hidden</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#ready-states"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>

// https://html.spec.whatwg.org/multipage/media.html#ready-states:eligible-for-autoplay-2

promise_test(async t => {
  let video = document.createElement("video");
  video.src = getVideoURI("/media/movie_5");
  video.autoplay = true;
  // In Safari, Chrome and Firefox, the video needs to be muted in order to be
  // paused when hidden. They decided to do this in order to save resources when
  // a video goes out of view and isn't expected to make any sound.
  video.muted = true;
  video.loop = true;
  let watcher = new EventWatcher(t, video, ["playing", "pause"]);
  document.body.appendChild(video);

  await watcher.wait_for("playing");
  assert_false(video.paused, "paused when video is display");
  video.hidden = true;

  await watcher.wait_for("pause");
  assert_true(video.paused, "paused when video is hidden");
  video.hidden = false;

  await watcher.wait_for("playing");
  assert_false(video.paused, "paused when video is display");
}, "Allow delaying autoplay until video elements become visible");

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
  "source_name": "html/semantics/embedded-content/media-elements/ready-states/autoplay-hidden.optional.html"
}
```
