# html/webappapis/timers/setinterval-settimeout-clamping.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/timers/setinterval-settimeout-clamping.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name="assert" content ="setTimeout and setInterval sequencing is correct even with 0 timeout">
<link rel="help" href="https://html.spec.whatwg.org/#run-steps-after-a-timeout" />
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<script>
async_test(t => {
  let done = false;
  const id = setInterval(() => {
    done = true;
  }, 0);
  t.add_cleanup(() => clearInterval(id));

  setTimeout(t.step_func(() => {
    assert_true(done);
    t.done();
  }), 0);
}, "setInterval(0) before setTimeout(0)");

async_test(t => {
  let done = false;
  setTimeout(() => {
    done = true;
  }, 0);

  const id = setInterval(t.step_func(() => {
    assert_true(done);
    t.done();
  }), 0);
  t.add_cleanup(() => clearInterval(id));
}, "setTimeout(0) before setInterval(0)");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/timers/setinterval-settimeout-clamping.html"
}
```
