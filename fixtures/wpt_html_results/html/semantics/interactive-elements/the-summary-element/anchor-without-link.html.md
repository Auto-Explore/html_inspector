# html/semantics/interactive-elements/the-summary-element/anchor-without-link.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-summary-element/anchor-without-link.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>summary element: clicking on anchor without link</title>
<link rel="author" title="Di Zhang" href="mailto:dizhangg@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/C/#the-summary-element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<details id="details">
  <summary><a id="no_inline">Details</a></summary>
  <p>Text</p>
</details>

<details id="details_inline">
  <summary><a><i id="has_inline">Details</i></a></summary>
  <p>Text</p>
</details>


<script>

async function testClickingOnAnchorWithoutLink (detailsId, targetId) {
  const details = document.getElementById(detailsId);
  const target = document.getElementById(targetId);
  const initialLoc = location.hash;

  assert_false(details.open);
  target.click();
  assert_true(details.open);
  assert_equals(location.hash, initialLoc);
}

promise_test(() => testClickingOnAnchorWithoutLink('details', 'no_inline'),
  "clicking on anchor without link should open details and not navigate.");

promise_test(() =>  testClickingOnAnchorWithoutLink('details_inline', 'has_inline'),
  "clicking on anchor without link, with embedded inline element should open details and not navigate.");

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
  "source_name": "html/semantics/interactive-elements/the-summary-element/anchor-without-link.html"
}
```
