# html/cross-origin-opener-policy/header-parsing-failures.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/header-parsing-failures.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name=timeout content=long>
<meta name="variant" content="?1-4">
<meta name="variant" content="?5-8">
<meta name="variant" content="?9-12">
<meta name="variant" content="?12-last">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/subset-tests.js"></script>
<script src="/common/utils.js"></script>
<script src="resources/common.js"></script>
<script src="resources/popup-test.js"></script>
<script>

[
  // None of the following should be recognized as "same-origin" (hence the
  // preserved opener).
  {
    "title": "coop with semi-column",
    "coop": "same-origin;",
  },
  {
    "title": "coop with vertical tab",
    "coop": "\u000bsame-origin\u000b",
  },
  {
    "title": "coop with form feed",
    "coop": "\u000csame-origin\u000c",
  },
  {
    "title": "coop with carriage return",
    "coop": "\u000dsame-origin\u000d",
  },
  {
    "title": "coop with capital letter",
    "coop": "Same-origin",
  },
  {
    "title": "coop with bad structured header 1",
    "coop": "same-origin;\tfoo=bar",
  },
  {
    "title": "coop with bad structured header 2",
    "coop": "same-origin ;foo=bar",
  },
  {
    "title": "coop with bad structured header 3",
    "coop": "same-origin; foo=bar;",
  },
  {
    "title": "coop as a structured header 'string' item",
    "coop": "\"same-origin\"",
  },
  {
    "title": "coop as a structured header 'byte sequence' item",
    "coop": ":c2FtZS1vcmlnaW4=:",
  },
  {
    "title": "coop as a structured header 'boolean' item",
    "coop": "?1",
  },
  {
    "title": "coop as a structured header 'integer or decimal' item",
    "coop": "1",
  },
  {
    "title": "coop as an unrecognized structured header type",
    "coop": "$same-origin",
  },
  {
    "title": "coop with duplicate value",
    "coop": "same-origin same-origin",
  },
  {
    // Note: comma must be escaped here to not mess with the WPT pipe function.
    "title": "coop with duplicate value, separated by a comma",
    "coop": "same-origin\\,same-origin",
  },
  {
    "title": "coop with preceding asterisk character",
    "coop": "*same-origin ",
  }
].forEach(variant => {
  subsetTest(popup_test, `Parsing ${variant.title}`, SAME_ORIGIN, { coop: variant.coop }, "preserved");
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
  "source_name": "html/cross-origin-opener-policy/header-parsing-failures.https.html"
}
```
