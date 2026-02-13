# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-scrolldelay.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-scrolldelay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: marquee-scrolldelay</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/multipage/obsolete.html#the-marquee-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<marquee id="test1" scrolldelay="aa">Test1</marquee>
<marquee id="test2" scrolldelay="-1">Test2</marquee>
<marquee id="test3" scrolldelay="1">Test3</marquee>
<marquee id="test4" scrolldelay="100">Test4</marquee>
<script>
  test(function() {
    var mq = document.getElementById("test1");
    assert_equals(mq.scrollDelay, 85, "The delay time should be 85ms.");
  }, "The scrolldelay attribute is a string");

  test(function() {
    var mq = document.getElementById("test2");
    assert_equals(mq.scrollDelay, 85, "The delay time should be 85ms.");
  }, "The scrolldelay attribute is a negative");

  test(function() {
    var mq = document.getElementById("test3");
    assert_equals(mq.scrollDelay, 1,
                  "The delay time should be 1ms (although this doesn't " +
                  "match rendering).");
  }, "The scrolldelay attribute is less than 60");

  test(function() {
    var mq = document.getElementById("test4");
    assert_equals(mq.scrollDelay, 100, "The delay time should be 100ms.");
  }, "The scrolldelay attribute is greater than 60");
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
        "byte_end": 420,
        "byte_start": 383,
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
        "byte_end": 473,
        "byte_start": 436,
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
        "byte_end": 525,
        "byte_start": 489,
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
        "byte_end": 579,
        "byte_start": 541,
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-scrolldelay.html"
}
```
