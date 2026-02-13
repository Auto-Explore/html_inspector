# html/semantics/embedded-content/media-elements/track/track-element/track-remove-by-setting-innerHTML.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-by-setting-innerHTML.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Removing a track by setting video.innerHTML doesn't crash</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track default src="resources/captions-gaps.vtt">
    <script>
        // https://bugs.webkit.org/show_bug.cgi?id=100981
        async_test(function(t) {
            var firstSeek = true;
            var video = document.querySelector('video');
            video.onseeked = t.step_func(function() {
                if (!firstSeek) {
                    t.done();
                    return;
                }

                // Remove the text track
                video.innerHTML = '';

                // Seek again to force a repaint.
                video.currentTime = 7.9;
                firstSeek = false;
            });

            video.currentTime = 0.5;
            video.src = getVideoURI('/media/counting');
        });
    </script>
</video>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-by-setting-innerHTML.html"
}
```
