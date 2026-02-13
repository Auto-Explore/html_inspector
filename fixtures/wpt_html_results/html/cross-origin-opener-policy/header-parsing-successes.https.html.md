# html/cross-origin-opener-policy/header-parsing-successes.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/header-parsing-successes.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="resources/common.js"></script>
<script src="resources/popup-test.js"></script>
<script>

[
  // All of the following should be recognized as "same-origin" (hence the
  // severed opener link).
  {
    "title": "coop with leading space",
    "coop": " same-origin",
  },
  {
    "title": "coop with trailing space",
    "coop": "same-origin ",
  },
  {
    "title": "coop with leading tab",
    "coop": "\tsame-origin",
  },
  {
    "title": "coop with trailing tab",
    "coop": "same-origin\t",
  },
  {
    "title": "coop with duplicate value, separated by semi-column",
    "coop": "same-origin;same-origin",
  },
  {
    "title": "coop with valid structured header",
    "coop": "same-origin; foo=bar",
  }
].forEach(variant => {
  popup_test(`Parsing ${variant.title}`, SAME_ORIGIN, { coop: variant.coop }, "severed");
});

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
  "source_name": "html/cross-origin-opener-policy/header-parsing-successes.https.html"
}
```
