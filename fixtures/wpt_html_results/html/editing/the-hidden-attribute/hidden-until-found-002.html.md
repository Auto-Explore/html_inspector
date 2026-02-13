# html/editing/the-hidden-attribute/hidden-until-found-002.html

Counts:
- errors: 5
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<meta charset="utf8">
<title>Tab order navigation ignores hidden=until-found subtrees</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">
<meta name="assert" content="tab order navigation ignores hidden=until-found subtrees.">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input id=one type=text></input>
<div hidden=until-found>
  <input id=two type=text></input>
  <input id=three type=text></input>
  <input id=four type=text></input>
</div>
<input id=five type=text></input>

<script>
async_test((t) => {
  const tab = "\ue004";
  async function step1() {
    await test_driver.send_keys(document.body, tab);
    t.step(() => {
      assert_equals(document.activeElement, document.getElementById("one"));
    });
    requestAnimationFrame(step2);
  }

  async function step2() {
    await test_driver.send_keys(document.body, tab);
    t.step(() => {
      assert_equals(document.activeElement, document.getElementById("five"));
    });
    t.done();
  }

  window.onload = () => { requestAnimationFrame(step1); };
}, "Tab order navigation skips hidden=until-found subtrees");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 683,
        "byte_start": 675,
        "col": 25,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 743,
        "byte_start": 735,
        "col": 27,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 780,
        "byte_start": 772,
        "col": 29,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 816,
        "byte_start": 808,
        "col": 28,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 857,
        "byte_start": 849,
        "col": 26,
        "line": 20
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-002.html"
}
```
