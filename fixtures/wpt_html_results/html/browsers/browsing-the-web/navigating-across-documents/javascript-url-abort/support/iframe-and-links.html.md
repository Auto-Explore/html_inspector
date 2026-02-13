# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/support/iframe-and-links.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/support/iframe-and-links.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<iframe name="iframe"></iframe>

<!-- slow link's response is delayed by 1 second -->
<!-- https://wptserve.readthedocs.io/en/latest/pipes.html#trickle -->
<a target="iframe" href="set-child-loaded.html?pipe=trickle(d1)" id="slowLink">slow link</a>
<a target="iframe" href="javascript:'javascript:string <script> parent.javascriptStringDocLoaded(); </script>'" id="javascriptStringLink">javascript:string link</a>
<a target="iframe" href="javascript:undefined" id="javascriptUndefinedLink">javascript:undefined link</a>

<script>
// set-child-loaded.html (the slow link) sets this to true.
window.childLoaded = false;

// Do nothing when the javascript:string doc has loaded, if it's correctly targeted to the above iframe.
// However, if it replaces this document, it needs to fail the test (handled in the parent).
function javascriptStringDocLoaded() {}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “javascript:'javascript:string <script> parent.javascriptStringDocLoaded(); </script>'” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 404,
        "byte_start": 266,
        "col": 1,
        "line": 8
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-abort/support/iframe-and-links.html"
}
```
