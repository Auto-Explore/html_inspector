# html/semantics/embedded-content/media-elements/video_volume_check.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/video_volume_check.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Video Test: video_volume_check</title>
    <link rel="author" title="Intel" href="http://www.intel.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-volume" />
    <meta name="flags" content="" />
    <meta name="assert" content="Check that video.volume returns the value of the muted content attribute" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <video id="m">The user agent doesn't support media element.</video>
    <script type="text/javascript">
        var media = document.getElementById("m");
        var VOLUME = {
            'SILENT'  :  0.0,
            'NORMAL'  :  0.5,
            'LOUDEST' :  1.0,
            'LOWER'   : -1.1,
            'UPPER'   :  1.1,
        };

        test(function() {
            assert_false(media.volume < VOLUME.SILENT || media.volume > VOLUME.LOUDEST, "media.volume outside the range 0.0 to 1.0 inclusive");
        }, "Check if the intial value of the video.volume is in the range 0.0 to 1.0 inclusive");

        function volume_setting(vol, name)
        {
            if (vol < VOLUME.SILENT || vol > VOLUME.LOUDEST) {
                try {
                    media.volume = vol;
                    test(function() {
                        assert_true(false, "media.volume setting exception");
                    }, name);
                } catch(e) {
                    test(function() {
                        // 1 should be e.IndexSizeError or e.INDEX_SIZE_ERR in previous spec
                        assert_equals(e.code, 1, "media.volume setting exception");
                    }, name);
                }
            } else {
                media.volume = vol;
                test(function() {
                    assert_equals(media.volume, vol, "media.volume new value");
                }, name);
            }
        }

        volume_setting(VOLUME.NORMAL,  "Check if video.volume is able to set to new value in the range 0.0 to 1.0");
        volume_setting(VOLUME.SILENT,  "Check if media.volume is able to set to new value 0.0 as silent");
        volume_setting(VOLUME.LOUDEST, "Check if media.volume is able to set to new value 1.0 as loudest");
        volume_setting(VOLUME.LOWER,   "Check if media.volume is set to new value less than 0.0 that expecting an IndexSizeError exception is to be thrown");
        volume_setting(VOLUME.UPPER,   "Check if video.volume is set to new value greater than 1.0 that expecting an IndexSizeError exception is to be thrown");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 650,
        "byte_start": 619,
        "col": 5,
        "line": 15
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/video_volume_check.html"
}
```
