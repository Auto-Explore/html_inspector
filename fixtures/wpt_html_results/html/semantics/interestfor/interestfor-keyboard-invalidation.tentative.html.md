# html/semantics/interestfor/interestfor-keyboard-invalidation.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-keyboard-invalidation.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<link rel=match href="interestfor-keyboard-invalidation-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<button id=b1 interestfor=target>Button</button>
<button id=b2 interestfor=target>Button</button>
<button id=b3 interestfor=target>Button</button>
<button id=b4>Button</button>
<button id=b5>Button</button>
<div popover id=target>Target</div>

<style>
  :interest-source {
    background-color: purple;
  }
  :interest-source:has-partial-interest {
    background-color: red;
  }
  /* Test complicated combinators: */
  :interest-source + button {
    background-color: green;
  }
  :root:has(:interest-source) #b5 {
    background-color: green;
  }
  :interest-target {
    background-color: yellow;
  }
  [interestfor] {
    interest-delay: 0s;
  }
  #target {
    inset:auto;
    top:50px;
    left:0;
  }
</style>

<script>
buttons = Array.from(document.querySelectorAll('[interestfor]'));
async function runTest() {
  for (const b of buttons) {
    b.focus();
  }
  document.documentElement.classList.remove("reftest-wait");
}
runTest();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 691,
        "byte_start": 684,
        "col": 1,
        "line": 18
      }
    },
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
  "source_name": "html/semantics/interestfor/interestfor-keyboard-invalidation.tentative.html"
}
```
