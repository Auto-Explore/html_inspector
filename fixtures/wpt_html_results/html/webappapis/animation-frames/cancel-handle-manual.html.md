# html/webappapis/animation-frames/cancel-handle-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/cancel-handle-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>AnimationTiming Test: cancelAnimationFrame used to cancel request callback</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#animation-frames">

<style>
  #animated {
    background: blue;
    color: white;
    height: 100px;
    width: 100px;
    position: absolute;
  }
</style>

<p>
  Test passes if there is a filled blue square with 'Filler Text',
  which moves from left to right repeatly, when click the 'stop' button,
  the square stops.
</p>
<button onclick="stop()">stop</button>
<div id="animated">Filler Text</div>

<script>

let requestId = 0;
let requestAnimation = window.requestAnimationFrame;
let cancelAnimation = window.cancelAnimationFrame;

function animate(time) {
  let div = document.getElementById("animated");
  div.style.left = (time - animationStartTime) % 2000 / 4 + "px";
  requestId = requestAnimation(animate);
}

function start() {
  animationStartTime = window.performance.now();
  requestId = requestAnimation(animate);
}

function stop() {
  if (requestId) {
    cancelAnimation(requestId);
    requestId = 0;
  }
}

start();

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
  "source_name": "html/webappapis/animation-frames/cancel-handle-manual.html"
}
```
