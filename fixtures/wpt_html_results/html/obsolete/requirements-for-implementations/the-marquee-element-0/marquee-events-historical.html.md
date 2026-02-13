# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-events-historical.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-events-historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Marquee events must not be implemented</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/obsolete.html#the-marquee-element">
<link rel="help" href="https://github.com/whatwg/html/pull/6343">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>
<marquee width="1" behavior="alternate">&nbsp;</marquee>
<marquee width="1">&nbsp;</marquee>
<marquee width="1" loop="2" behavior="alternate">&nbsp;</marquee>
<marquee width="1" loop="2">&nbsp;</marquee>

<script>
test(() => {
  assert_false("onstart" in HTMLMarqueeElement.prototype, "onstart");
  assert_false("onfinish" in HTMLMarqueeElement.prototype, "onfinish");
  assert_false("onbounce" in HTMLMarqueeElement.prototype, "onbounce");
}, "Event handler IDL attributes must not be implemented");

// Because we use width="1" they will bounce and finish really fast
async_test(t => {
  for (const m of document.querySelectorAll("marquee")) {
    m.addEventListener("start", t.unreached_func(`start: ${m.outerHTML}`));
    m.addEventListener("finish", t.unreached_func(`finish: ${m.outerHTML}`));
    m.addEventListener("bounce", t.unreached_func(`bounce: ${m.outerHTML}`));
  }

  t.step_timeout(() => t.done(), 100);
}, "No events must be fired, at least during the first 100 ms");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 435,
        "byte_start": 395,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 471,
        "byte_start": 452,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 537,
        "byte_start": 488,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 582,
        "byte_start": 554,
        "col": 1,
        "line": 13
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-events-historical.html"
}
```
