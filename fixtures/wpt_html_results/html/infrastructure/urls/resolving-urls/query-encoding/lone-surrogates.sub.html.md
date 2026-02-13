# html/infrastructure/urls/resolving-urls/query-encoding/lone-surrogates.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/resolving-urls/query-encoding/lone-surrogates.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset={{GET[encoding]}}> <!-- ends up as <meta charset> by default which is windows-1252 -->
<meta name=variant content="?encoding=windows-1252">
<meta name=variant content="?encoding=utf8">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
function expected(encoding) {
  return "?" + {
    // Replacement character (not bogus UTF-8)
    "UTF-8": "%EF%BF%BD",
    // Charref for the replacement character (not the lone surrogate)
    "windows-1252": "%26%2365533%3B",
  }[encoding];
}

test(t => {
  const elm = document.createElement("a");
  document.body.appendChild(elm);
  t.add_cleanup(() => elm.remove());
  elm.setAttribute("href", "?\uD800");

  const shouldEndWith = expected(document.characterSet);
  assert_true(
    elm.href.endsWith(shouldEndWith),
    `${elm.href} did not end with ${shouldEndWith}`
  );
}, `Query parsing with lone surrogates in ${document.characterSet}`);
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “{{GET[encoding]}}” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 48,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/infrastructure/urls/resolving-urls/query-encoding/lone-surrogates.sub.html"
}
```
