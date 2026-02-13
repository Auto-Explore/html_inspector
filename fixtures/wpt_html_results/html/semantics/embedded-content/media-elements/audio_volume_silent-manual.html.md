# html/semantics/embedded-content/media-elements/audio_volume_silent-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/audio_volume_silent-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Audio Test: audio_volume_silent</title>
    <link rel="author" title="Intel" href="http://www.intel.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-volume" />
    <meta name="flags" content="" />
    <meta name="assert" content="Check if the volume attribute is set to 0.0 as silent in the audio element that expecting the user hears no sound" />
    <script src="/common/media.js"></script>
  </head>
  <body>
    <p>Test passes if the audio is playing without sound heard and the text 'The user agent doesn't support media element.' does not appear anywhere on this page</p>
    <audio id="m" controls volume=0.0>The user agent doesn't support media element.</audio>
    <script type="text/javascript">
        var media = document.getElementById("m");
        media.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
        media.volume = 0.0;
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
        "byte_end": 784,
        "byte_start": 753,
        "col": 5,
        "line": 14
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
  "source_name": "html/semantics/embedded-content/media-elements/audio_volume_silent-manual.html"
}
```
