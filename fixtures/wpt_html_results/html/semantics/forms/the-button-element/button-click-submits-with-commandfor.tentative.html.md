# html/semantics/forms/the-button-element/button-click-submits-with-commandfor.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-click-submits-with-commandfor.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<title>Clicking a button should submit the form</title>
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<link
  rel="help"
  href="https://html.spec.whatwg.org/multipage/#attr-button-type-submit-state"
/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<form id="form">
  <button id="button"></button>
</form>

<script>
  const form = document.getElementById("form");
  const button = document.getElementById("button");

  function resetState() {
    button.removeAttribute("commandfor");
    button.removeAttribute("command");
    button.removeAttribute("disabled");
    button.removeAttribute("form");
    button.removeAttribute("type");
  }

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("command", "--foo");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_false(called, "submit should not have been dispatched");
  }, "clicking a button (implicit type) should NOT trigger a submit (with command attribute)");

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("commandfor", "whatever");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_false(called, "submit should not have been dispatched");
  }, "clicking a button (implicit type) should NOT trigger a submit (with commandfor attribute)");

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("command", "--foo");
    button.setAttribute("commandfor", "whatever");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_false(called, "submit should not have been dispatched");
  }, "clicking a button (implicit type) should NOT trigger a submit (with command and commandfor attribute)");

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("command", "--foo");
    button.setAttribute("type", "submit");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_true(called, "submit should have been dispatched");
  }, "clicking a button (explicit type=submit) SHOULD trigger a submit (with command attribute)");

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("commandfor", "whatever");
    button.setAttribute("type", "submit");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_true(called, "submit should have been dispatched");
  }, "clicking a button (explicit type=submit) SHOULD trigger a submit (with commandfor attribute)");

  test((t) => {
    t.add_cleanup(resetState);
    button.setAttribute("command", "--foo");
    button.setAttribute("commandfor", "whatever");
    button.setAttribute("type", "submit");

    let called = false;
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      called = true;
    }, { once: true });
    button.click();
    assert_true(called, "submit should have been dispatched");
  }, "clicking a button (explicit type=submit) SHOULD trigger a submit (with command and commandfor attribute)");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 176,
        "byte_start": 97,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 176,
        "byte_start": 97,
        "col": 1,
        "line": 4
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
  "source_name": "html/semantics/forms/the-button-element/button-click-submits-with-commandfor.tentative.html"
}
```
