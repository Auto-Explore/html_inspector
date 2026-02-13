# html/semantics/embedded-content/media-elements/video_loop_base.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/video_loop_base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Video Test: video_loop_base</title>
    <link rel="author" title="Intel" href="http://www.intel.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-loop" />
    <meta name="flags" content="" />
    <meta name="assert" content="Check if video.loop is set to true that expecting the seeking event is fired more than once" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/common/media.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <video id="m" controls>The user agent doesn't support media element.</video>
    <script type="text/javascript">
        var media = document.getElementById("m");
        var name = document.getElementsByName("assert")[0].content;
        var t = async_test(name);
        var looped = false;

        function startTest() {
            if (looped) {
                t.step(function() {
                    assert_true(true, "looped");
                });
                t.done();
                media.pause();
            }

            looped = true;
        }

        media.addEventListener("seeking", startTest, false);
        media.loop = true;
        media.src = getVideoURI("/media/2x2-green") + "?" + new Date() + Math.random();
        media.play();
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
        "byte_end": 718,
        "byte_start": 687,
        "col": 5,
        "line": 16
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
  "source_name": "html/semantics/embedded-content/media-elements/video_loop_base.html"
}
```
