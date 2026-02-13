# html/semantics/embedded-content/media-elements/track/track-element/track-selection-task-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-selection-task-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLTrackElement Text Track Selection Task Order</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
/**
 * This test is used to ensure that we queue 'honor user preferences for automatic
 * text track selection' as a macro task, not a micro task. In this test, we
 * trigger a media event before queuing a text track selection task, and check
 * the text track's mode to know whether the text track selection runs after the
 * task for media event.
 */
async_test(function(t) {
    let video = document.createElement("video");
    video.play();
    video.onplay = t.step_func(startedPlay);

    // When we create a text track element, it queue a task to run automatic
    // text track selection later.
    let track = document.createElement("track");
    track.default = true;
    video.appendChild(track);
    assert_equals(track.track.mode, "disabled", "Text track's mode is disabled by default.");

    function startedPlay() {
        assert_equals(track.track.mode, "disabled", "Text track selection hasn't started yet.");
        track.onerror = t.step_func_done(trackError);
    }

    function trackError() {
        assert_equals(track.track.mode, "showing", "Text track selection modified track's mode.");
        t.done();
    }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-selection-task-order.html"
}
```
