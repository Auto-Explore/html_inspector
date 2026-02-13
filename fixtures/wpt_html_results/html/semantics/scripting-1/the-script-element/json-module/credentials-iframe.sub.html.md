# html/semantics/scripting-1/the-script-element/json-module/credentials-iframe.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/credentials-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">

<script type="module">
    import json from "./cross-origin.py?id=sameOriginNoneDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
    window.sameOriginNoneDescendant = json.requestHadCookies;
</script>
<script type="module" crossOrigin="anonymous">
    import json from "./cross-origin.py?id=sameOriginAnonymousDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
    window.sameOriginAnonymousDescendant = json.requestHadCookies;
</script>
<script type="module" crossOrigin="use-credentials">
    import json from "./cross-origin.py?id=sameOriginUseCredentialsDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
    window.sameOriginUseCredentialsDescendant = json.requestHadCookies;
</script>
<script type="module">
    import json from "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/json-module/cross-origin.py?id=crossOriginNoneDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
    window.crossOriginNoneDescendant = json.requestHadCookies;
</script>
<script type="module" crossOrigin="anonymous">
    import json from "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/json-module/cross-origin.py?id=crossOriginAnonymousDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
    window.crossOriginAnonymousDescendant = json.requestHadCookies;
</script>
<script type="module" crossOrigin="use-credentials">
import json from "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/json-module/cross-origin.py?id=crossOriginUseCredentialsDescendant&origin=http://{{host}}:{{ports[http][0]}}" with { type: "json" };
window.crossOriginUseCredentialsDescendant = json.requestHadCookies;
</script>

<script type="text/javascript">
window.addEventListener('load', event => {
  window.parent.postMessage({}, '*');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1912,
        "byte_start": 1881,
        "col": 1,
        "line": 29
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/credentials-iframe.sub.html"
}
```
