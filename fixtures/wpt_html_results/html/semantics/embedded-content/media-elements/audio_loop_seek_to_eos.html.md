# html/semantics/embedded-content/media-elements/audio_loop_seek_to_eos.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/audio_loop_seek_to_eos.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title>Seeking to the end of looping audio</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/media.js"></script>
</head>
<body>
<div id="log"></div>
<audio id="a" controls loop></audio>
<script type="text/javascript">

promise_test(async t => {
  const a = document.getElementById("a");
  a.src = getAudioURI("/media/sound_5") + "?" + new Date() + Math.random();
  await a.play();

  // Seek to the end of audio (EOS). However, as audio is looping, it should
  // keep playing after seeking.
  a.currentTime = a.duration;
  await new Promise(r => a.onseeked = r);
  await new Promise(r => a.ontimeupdate = r);
  assert_false(a.paused);
} , "seeking to the end of looping audio");

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
        "byte_end": 340,
        "byte_start": 309,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/embedded-content/media-elements/audio_loop_seek_to_eos.html"
}
```
