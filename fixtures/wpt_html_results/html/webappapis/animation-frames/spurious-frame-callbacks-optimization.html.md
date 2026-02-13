# html/webappapis/animation-frames/spurious-frame-callbacks-optimization.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/spurious-frame-callbacks-optimization.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Optimization of requestAnimationFrame callbacks that don't modify the DOM shouldn't break animations</title>
  <link rel="author" title="Mukilan Thiyagarajan" href="mailto:mukilan@igalia.com">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <div id="target">0</div>
</body>
<script>
"use strict";
async_test(function(test) {
  let frame = 0;
  const draw = (t) => {
    frame += 1;
    if (frame < 11) {
      // Don't mutate the DOM for 10 frames to meet the threshold for Servo's
      // spurious frame optimization to kick in.
      requestAnimationFrame(draw);
    } else if (frame == 11) {
      // Don't schedule next rAF so the compositor's tick is disabled.
      // This is specific to Servo as the spurious frame detection at the
      // time of this test was broken.
      test.step_timeout(() => {
        requestAnimationFrame(draw);
      }, 0);
    } else {
      // Normal frames.
      document.getElementById('target').innerText = t;
      requestAnimationFrame(draw);
    }
  };

  let target = document.getElementById('target');
  test.step_timeout(test.step_func_done(() => {
    assert_greater_than(parseInt(target.innerText), 500);
  }), 550);
  requestAnimationFrame(draw);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 395,
        "byte_start": 387,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1317,
        "byte_start": 395,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1326,
        "byte_start": 1317,
        "col": 1,
        "line": 41
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
  "source_name": "html/webappapis/animation-frames/spurious-frame-callbacks-optimization.html"
}
```
