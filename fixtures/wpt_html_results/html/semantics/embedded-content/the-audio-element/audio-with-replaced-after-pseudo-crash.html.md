# html/semantics/embedded-content/the-audio-element/audio-with-replaced-after-pseudo-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-audio-element/audio-with-replaced-after-pseudo-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="test-wait">
<meta charset="utf-8">
<title>HTML5 Media Elements: An 'audio' element with a replaced ::after shouldn't crash</title>
<link rel="author" title="Oriol Brufau" href="obrufau@igalia.com">
<link rel="help" href="https://github.com/servo/servo/issues/41183">

<style>
audio::after {
  content: url("/css/support/60x60-red.png");
}
</style>
<audio controls></audio>
<script src="/common/media.js"></script>
<script src="/common/rendering-utils.js"></script>
<script>
(async function() {
  const audio = document.querySelector("audio");
  audio.src = getAudioURI("/media/sound_5");
  for (let i = 0; i < 10; ++i) {
    await waitForAtLeastOneFrame();
    await audio.play();
    document.body.style.color = "cyan";
    await waitForAtLeastOneFrame();
    audio.pause();
    await waitForAtLeastOneFrame();
    document.body.style.color = "magenta";
  }
  document.documentElement.removeAttribute("class");
})();
</script>
</html>
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
  "source_name": "html/semantics/embedded-content/the-audio-element/audio-with-replaced-after-pseudo-crash.html"
}
```
