# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-loop.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-loop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: marquee-loop</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/obsolete.html#the-marquee-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<marquee id="test1" loop="a">Test1</marquee>
<marquee id="test2" loop="-2">Test2</marquee>
<marquee id="test3" loop="2">Test3</marquee>
<script>
  test(function() {
    var mq = document.getElementById("test1");
    assert_equals(mq.loop, -1, "The value of loop should be -1.");
  }, "marquee_loop_string");

  test(function() {
    var mq = document.getElementById("test2");
    assert_equals(mq.loop, -1, "The value of loop should be -1.");
  }, "marquee_loop_less_than_1");

  test(function() {
    var mq = document.getElementById("test3");
    assert_equals(mq.loop, 2, "The value of loop should be 2.");
  }, "marquee_loop_normal");
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
        "byte_end": 405,
        "byte_start": 376,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.element.marquee.obsolete",
      "message": "The “marquee” element is obsolete. Use CSS instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 421,
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
        "byte_end": 496,
        "byte_start": 467,
        "col": 1,
        "line": 11
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-loop.html"
}
```
