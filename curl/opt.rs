
pub enum CURLoption {
    /* This is the FILE * or void * the regular output should be written to. */
    WRITEDATA = 10001,

    //FILE = 10001,

    /* The full URL to get/put */
    URL = 10002,

    /* Port number to connect to, if other than default. */
    PORT = 3,

    /* Name of proxy to use. */
    PROXY = 10004,

    /* "name:password" to use when fetching. */
    USERPWD = 10005,

    /* "name:password" to use with proxy. */
    PROXYUSERPWD = 10006,

    /* Range to get, specified as an ASCII string. */
    RANGE = 10007,

    /* not used */

    /* Specified file stream to upload from (use as input;: */
    INFILE = 10009,

    /* Buffer to receive error messages in, must be at least CURL_ERROR_SIZE
     * bytes big. If this is not used, error messages go to stderr instead: */
    ERRORBUFFER = 10010,

    /* Function that will be called to store the output (instead of fwrite;. The
     * parameters will use fwrite(; syntax, make sure to follow them. */
    WRITEFUNCTION = 20011,

    /* Function that will be called to read the input (instead of fread;. The
     * parameters will use fread(; syntax, make sure to follow them. */
    READFUNCTION = 20012,

    /* Time-out the read operation after this amount of seconds */
    TIMEOUT = 13,

    /* If the CURLOPT_INFILE is used, this can be used to inform libcurl about
     * how large the file being sent really is. That allows better error
     * checking and better verifies that the upload was successful. -1 means
     * unknown size.
     *
     * For large file support, there is also a _LARGE version of the key
     * which takes an off_t type, allowing platforms with larger off_t
     * sizes to handle larger files.  See below for INFILESIZE_LARGE.
     */
    INFILESIZE = 14,

    /* POST static input fields. */
    POSTFIELDS = 10015,

    /* Set the referrer page (needed by some CGIs; */
    REFERER = 10016,

    /* Set the FTP PORT string (interface name, named or numerical IP address;
         Use i.e '-' to use default address. */
    FTPPORT = 10017,

    /* Set the User-Agent string (examined by some CGIs; */
    USERAGENT = 10018,

    /* If the download receives less than "low speed limit" bytes/second
     * during "low speed time" seconds, the operations is aborted.
     * You could i.e if you have a pretty high speed connection, abort if
     * it is less than 2000 bytes/sec during 20 seconds.
     */

    /* Set the "low speed limit" */
    LOW_SPEED_LIMIT = 19,

    /* Set the "low speed time" */
    LOW_SPEED_TIME = 20,

    /* Set the continuation offset.
     *
     * Note there is also a _LARGE version of this key which uses
     * off_t types, allowing for large file offsets on platforms which
     * use larger-than-32-bit off_t's.  Look below for RESUME_FROM_LARGE.
     */
    RESUME_FROM = 21,

    /* Set cookie in request: */
    COOKIE = 10022,

    /* This points to a linked list of headers, struct curl_slist kind */
    HTTPHEADER = 10023,

    /* This points to a linked list of post entries, struct curl_httppost */
    HTTPPOST = 10024,

    /* name of the file keeping your private SSL-certificate */
    SSLCERT = 10025,

    /* password for the SSL or SSH private key */
    KEYPASSWD = 10026,

    /* send TYPE parameter? */
    CRLF = 27,

    /* send linked-list of QUOTE commands */
    QUOTE = 10028,

    /* send FILE * or void * to store headers to, if you use a callback it
         is simply passed to the callback unmodified */
    HEADERDATA = 10029,

    //WRITEHEADER = 10029,

    /* point to a file to read the initial cookies from, also enables
         "cookie awareness" */
    COOKIEFILE = 10031,

    /* What version to specifically try to use.
         See CURL_SSLVERSION defines below. */
    SSLVERSION = 32,

    /* What kind of HTTP time condition to use, see defines */
    TIMECONDITION = 33,

    /* Time to use with the above condition. Specified in number of seconds
         since 1 Jan 1970 */
    TIMEVALUE = 34,

    /* 35 = OBSOLETE */

    /* Custom request, for customizing the get command like
         HTTP: DELETE, TRACE and others
         FTP: to use a different list command
         */
    CUSTOMREQUEST = 10036,

    /* HTTP request, for odd commands like DELETE, TRACE and others */
    STDERR = 10037,

    /* 38 is not used */

    /* send linked-list of post-transfer QUOTE commands */
    POSTQUOTE = 10039,

    WRITEINFO = 10040, /* DEPRECATED, do not use! */

    VERBOSE = 41,      /* talk a lot */
    HEADER = 42,       /* throw the header out too */
    NOPROGRESS = 43,   /* shut off the progress meter */
    NOBODY = 44,       /* use HEAD to get http document */
    FAILONERROR = 45,  /* no output on http error codes >= 300 */
    UPLOAD = 46,       /* this is an upload */
    POST = 47,         /* HTTP POST method */
    DIRLISTONLY = 48,  /* bare names when listing directories */

    APPEND = 50,       /* Append instead of overwrite on upload! */

    /* Specify whether to read the user+password from the .netrc or the URL.
     * This must be one of the CURL_NETRC_* enums below. */
    NETRC = 51,

    FOLLOWLOCATION = 52,  /* use Location: Luke! */

    TRANSFERTEXT = 53, /* transfer data in text/ASCII format */
    PUT = 54,          /* HTTP PUT */

    /* 55 = OBSOLETE */

    /* Function that will be called instead of the internal progress display
     * function. This function should be defined as the curl_progress_callback
     * prototype defines. */
    PROGRESSFUNCTION = 20056,

    /* Data passed to the progress callback */
    PROGRESSDATA = 10057,

    /* We want the referrer field set automatically when following locations */
    AUTOREFERER = 58,

    /* Port of the proxy, can be set in the proxy string as well with:
         "[host]:[port]" */
    PROXYPORT = 59,

    /* size of the POST input data, if strlen(; is not good to use */
    POSTFIELDSIZE = 60,

    /* tunnel non-http operations through a HTTP proxy */
    HTTPPROXYTUNNEL = 61,

    /* Set the interface string to use as outgoing network interface */
    INTERFACE = 10062,

    /* Set the krb4/5 security level, this also enables krb4/5 awareness.  This
     * is a string, 'clear', 'safe', 'confidential' or 'private'.  If the string
     * is set but doesn't match one of these, 'private' will be used.  */
    KRBLEVEL = 10063,

    /* Set if we should verify the peer in ssl handshake, set 1 to verify. */
    SSL_VERIFYPEER = 64,

    /* The CApath or CAfile used to validate the peer certificate
         this option is used only if SSL_VERIFYPEER is true */
    CAINFO = 10065,

    /* 66 = OBSOLETE */
    /* 67 = OBSOLETE */

    /* Maximum number of http redirects to follow */
    MAXREDIRS = 68,

    /* Pass a long set to 1 to get the date of the requested document (if
         possible;! Pass a zero to shut it off. */
    FILETIME = 69,

    /* This points to a linked list of telnet options */
    TELNETOPTIONS = 10070,

    /* Max amount of cached alive connections */
    MAXCONNECTS = 71,

    CLOSEPOLICY = 72, /* DEPRECATED, do not use! */

    /* 73 = OBSOLETE */

    /* Set to explicitly use a new connection for the upcoming transfer.
         Do not use this unless you're absolutely sure of this, as it makes the
         operation slower and is less friendly for the network. */
    FRESH_CONNECT = 74,

    /* Set to explicitly forbid the upcoming transfer's connection to be re-used
         when done. Do not use this unless you're absolutely sure of this, as it
         makes the operation slower and is less friendly for the network. */
    FORBID_REUSE = 75,

    /* Set to a file name that contains random data for libcurl to use to
         seed the random engine when doing SSL connects. */
    RANDOM_FILE = 10076,

    /* Set to the Entropy Gathering Daemon socket pathname */
    EGDSOCKET = 10077,

    /* Time-out connect operations after this amount of seconds, if connects are
         OK within this time, then fine... This only aborts the connect phase. */
    CONNECTTIMEOUT = 78,

    /* Function that will be called to store headers (instead of fwrite;. The
     * parameters will use fwrite(; syntax, make sure to follow them. */
    HEADERFUNCTION = 20079,

    /* Set this to force the HTTP request to get back to GET. Only really usable
         if POST, PUT or a custom request have been used first.
     */
    HTTPGET = 80,

    /* Set if we should verify the Common name from the peer certificate in ssl
     * handshake, set 1 to check existence, 2 to ensure that it matches the
     * provided hostname. */
    SSL_VERIFYHOST = 81,

    /* Specify which file name to write all known cookies in after completed
         operation. Set file name to "-" (dash; to make it go to stdout. */
    COOKIEJAR = 10082,

    /* Specify which SSL ciphers to use */
    SSL_CIPHER_LIST = 10083,

    /* Specify which HTTP version to use! This must be set to one of the
         CURL_HTTP_VERSION* enums set below. */
    HTTP_VERSION = 84,

    /* Specifically switch on or off the FTP engine's use of the EPSV command. By
         default, that one will always be attempted before the more traditional
         PASV command. */
    FTP_USE_EPSV = 85,

    /* type of the file keeping your SSL-certificate ("DER", "PEM", "ENG"; */
    SSLCERTTYPE = 10086,

    /* name of the file keeping your private SSL-key */
    SSLKEY = 10087,

    /* type of the file keeping your private SSL-key ("DER", "PEM", "ENG"; */
    SSLKEYTYPE = 10088,

    /* crypto engine for the SSL-sub system */
    SSLENGINE = 10089,

    /* set the crypto engine for the SSL-sub system as default
         the param has no meaning...
     */
    SSLENGINE_DEFAULT = 90,

    /* Non-zero value means to use the global dns cache */
    DNS_USE_GLOBAL_CACHE = 91, /* DEPRECATED, do not use! */

    /* DNS cache timeout */
    DNS_CACHE_TIMEOUT = 92,

    /* send linked-list of pre-transfer QUOTE commands */
    PREQUOTE = 10093,

    /* set the debug function */
    DEBUGFUNCTION = 20094,

    /* set the data for the debug function */
    DEBUGDATA = 10095,

    /* mark this as start of a cookie session */
    COOKIESESSION = 96,

    /* The CApath directory used to validate the peer certificate
         this option is used only if SSL_VERIFYPEER is true */
    CAPATH = 10097,

    /* Instruct libcurl to use a smaller receive buffer */
    BUFFERSIZE = 98,

    /* Instruct libcurl to not use any signal/alarm handlers, even when using
         timeouts. This option is useful for multi-threaded applications.
         See libcurl-the-guide for more background information. */
    NOSIGNAL = 99,

    /* Provide a CURLShare for mutexing non-ts data */
    SHARE = 10100,

    /* indicates type of proxy. accepted values are CURLPROXY_HTTP (default;
         CURLPROXY_SOCKS4, CURLPROXY_SOCKS4A and CURLPROXY_SOCKS5. */
    PROXYTYPE = 101,

    /* Set the Accept-Encoding string. Use this to tell a server you would like
         the response to be compressed. Before 7.21.6, this was known as
         CURLOPT_ENCODING */
    ACCEPT_ENCODING = 10102,

    /* Set pointer to private data */
    PRIVATE = 10103,

    /* Set aliases for HTTP 200 in the HTTP Response header */
    HTTP200ALIASES = 10104,

    /* Continue to send authentication (user+password; when following locations,
         even when hostname changed. This can potentially send off the name
         and password to whatever host the server decides. */
    UNRESTRICTED_AUTH = 105,

    /* Specifically switch on or off the FTP engine's use of the EPRT command (
         it also disables the LPRT attempt;. By default, those ones will always be
         attempted before the good old traditional PORT command. */
    FTP_USE_EPRT = 106,

    /* Set this to a bitmask value to enable the particular authentications
         methods you like. Use this in combination with CURLOPT_USERPWD.
         Note that setting multiple bits may cause extra network round-trips. */
    HTTPAUTH = 107,

    /* Set the ssl context callback function, currently only for OpenSSL ssl_ctx
         in second argument. The function must be matching the
         curl_ssl_ctx_callback proto. */
    SSL_CTX_FUNCTION = 20108,

    /* Set the userdata for the ssl context callback function's third
         argument */
    SSL_CTX_DATA = 10109,

    /* FTP Option that causes missing dirs to be created on the remote server.
         In 7.19.4 we introduced the convenience enums for this option using the
         CURLFTP_CREATE_DIR prefix.
    */
    FTP_CREATE_MISSING_DIRS = 110,

    /* Set this to a bitmask value to enable the particular authentications
         methods you like. Use this in combination with CURLOPT_PROXYUSERPWD.
         Note that setting multiple bits may cause extra network round-trips. */
    PROXYAUTH = 111,

    /* FTP option that changes the timeout, in seconds, associated with
         getting a response.  This is different from transfer timeout time and
         essentially places a demand on the FTP server to acknowledge commands
         in a timely manner. */
    FTP_RESPONSE_TIMEOUT = 112,

    //pub static SERVER_RESPONSE_TIMEOUT: i32 = FTP_RESPONSE_TIMEOUT;

    /* Set this option to one of the CURL_IPRESOLVE_* defines (see below; to
         tell libcurl to resolve names to those IP versions only. This only has
         affect on systems with support for more than one, i.e IPv4 _and_ IPv6. */
    IPRESOLVE = 113,

    /* Set this option to limit the size of a file that will be downloaded from
         an HTTP or FTP server.

         Note there is also _LARGE version which adds large file support for
         platforms which have larger off_t sizes.  See MAXFILESIZE_LARGE below. */
    MAXFILESIZE = 114,

    /* See the comment for INFILESIZE above, but in short, specifies
     * the size of the file being uploaded.  -1 means unknown.
     */
    INFILESIZE_LARGE = 30115,

    /* Sets the continuation offset.  There is also a LONG version of this;
     * look above for RESUME_FROM.
     */
    RESUME_FROM_LARGE = 30116,

    /* Sets the maximum size of data that will be downloaded from
     * an HTTP or FTP server.  See MAXFILESIZE above for the LONG version.
     */
    MAXFILESIZE_LARGE = 30117,

    /* Set this option to the file name of your .netrc file you want libcurl
         to parse (using the CURLOPT_NETRC option;. If not set, libcurl will do
         a poor attempt to find the user's home directory and check for a .netrc
         file in there. */
    NETRC_FILE = 10118,

    /* Enable SSL/TLS for FTP, pick one of:
         CURLUSESSL_TRY     - try using SSL, proceed anyway otherwise
         CURLUSESSL_CONTROL - SSL for the control connection or fail
         CURLUSESSL_ALL     - SSL for all communication or fail
    */
    USE_SSL = 119,

    /* The _LARGE version of the standard POSTFIELDSIZE option */
    POSTFIELDSIZE_LARGE = 30120,

    /* Enable/disable the TCP Nagle algorithm */
    TCP_NODELAY = 121,

    /* 122 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
    /* 123 OBSOLETE. Gone in 7.16.0 */
    /* 124 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
    /* 125 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
    /* 126 OBSOLETE, used in 7.12.3. Gone in 7.13.0 */
    /* 127 OBSOLETE. Gone in 7.16.0 */
    /* 128 OBSOLETE. Gone in 7.16.0 */

    /* When FTP over SSL/TLS is selected (with CURLOPT_USE_SSL; this option
         can be used to change libcurl's default action which is to first try
         "AUTH SSL" and then "AUTH TLS" in this order, and proceed when a OK
         response has been received.

         Available parameters are:
         CURLFTPAUTH_DEFAULT - let libcurl decide
         CURLFTPAUTH_SSL     - try "AUTH SSL" first, then TLS
         CURLFTPAUTH_TLS     - try "AUTH TLS" first, then SSL
    */
    FTPSSLAUTH = 129,

    IOCTLFUNCTION = 20130,
    IOCTLDATA = 10131,

    /* 132 OBSOLETE. Gone in 7.16.0 */
    /* 133 OBSOLETE. Gone in 7.16.0 */

    /* zero terminated string for pass on to the FTP server when asked for
         "account" info */
    FTP_ACCOUNT = 10134,

    /* feed cookies into cookie engine */
    COOKIELIST = 10135,

    /* ignore Content-Length */
    IGNORE_CONTENT_LENGTH = 136,

    /* Set to non-zero to skip the IP address received in a 227 PASV FTP server
         response. Typically used for FTP-SSL purposes but is not restricted to
         that. libcurl will then instead use the same IP address it used for the
         control connection. */
    FTP_SKIP_PASV_IP = 137,

    /* Select "file method" to use when doing FTP, see the curl_ftpmethod
         above. */
    FTP_FILEMETHOD = 138,

    /* Local port number to bind the socket to */
    LOCALPORT = 139,

    /* Number of ports to try, including the first one set with LOCALPORT.
         Thus, setting it to 1 will make no additional attempts but the first.
    */
    LOCALPORTRANGE = 140,

    /* no transfer, set up connection and let application use the socket by
         extracting it with CURLINFO_LASTSOCKET */
    CONNECT_ONLY = 141,

    /* Function that will be called to convert from the
         network encoding (instead of using the iconv calls in libcurl; */
    CONV_FROM_NETWORK_FUNCTION = 20142,

    /* Function that will be called to convert to the
         network encoding (instead of using the iconv calls in libcurl; */
    CONV_TO_NETWORK_FUNCTION = 20143,

    /* Function that will be called to convert from UTF8
         (instead of using the iconv calls in libcurl;
         Note that this is used only for SSL certificate processing */
    CONV_FROM_UTF8_FUNCTION = 20144,

    /* if the connection proceeds too quickly then need to slow it down */
    /* limit-rate: maximum number of bytes per second to send or receive */
    MAX_SEND_SPEED_LARGE = 30145,
    MAX_RECV_SPEED_LARGE = 30146,

    /* Pointer to command string to send if USER/PASS fails. */
    FTP_ALTERNATIVE_TO_USER = 10147,

    /* callback function for setting socket options */
    SOCKOPTFUNCTION = 20148,
    SOCKOPTDATA = 10149,

    /* set to 0 to disable session ID re-use for this transfer, default is
         enabled (== 1; */
    SSL_SESSIONID_CACHE = 150,

    /* allowed SSH authentication methods */
    SSH_AUTH_TYPES = 151,

    /* Used by scp/sftp to do public/private key authentication */
    SSH_PUBLIC_KEYFILE = 10152,
    SSH_PRIVATE_KEYFILE = 10153,

    /* Send CCC (Clear Command Channel; after authentication */
    FTP_SSL_CCC = 154,

    /* Same as TIMEOUT and CONNECTTIMEOUT, but with ms resolution */
    TIMEOUT_MS = 155,
    CONNECTTIMEOUT_MS = 156,

    /* set to zero to disable the libcurl's decoding and thus pass the raw body
         data to the application even when it is encoded/compressed */
    HTTP_TRANSFER_DECODING = 157,
    HTTP_CONTENT_DECODING = 158,

    /* Permission used when creating new files and directories on the remote
         server for protocols that support it, SFTP/SCP/FILE */
    NEW_FILE_PERMS = 159,
    NEW_DIRECTORY_PERMS = 160,

    /* Set the behaviour of POST when redirecting. Values must be set to one
         of CURL_REDIR* defines below. This used to be called CURLOPT_POST301 */
    POSTREDIR = 161,

    /* used by scp/sftp to verify the host's public key */
    SSH_HOST_PUBLIC_KEY_MD5 = 10162,

    /* Callback function for opening socket (instead of socket(2;;. Optionally,
         callback is able change the address or refuse to connect returning
         CURL_SOCKET_BAD.  The callback should have type
         curl_opensocket_callback */
    OPENSOCKETFUNCTION = 20163,
    OPENSOCKETDATA = 10164,

    /* POST volatile input fields. */
    COPYPOSTFIELDS = 10165,

    /* set transfer mode (;type=<a|i>; when doing FTP via an HTTP proxy */
    PROXY_TRANSFER_MODE = 166,

    /* Callback function for seeking in the input stream */
    SEEKFUNCTION = 20167,
    SEEKDATA = 10168,

    /* CRL file */
    CRLFILE = 10169,

    /* Issuer certificate */
    ISSUERCERT = 10170,

    /* (IPv6; Address scope */
    ADDRESS_SCOPE = 171,

    /* Collect certificate chain info and allow it to get retrievable with
         CURLINFO_CERTINFO after the transfer is complete. (Unfortunately; only
         working with OpenSSL-powered builds. */
    CERTINFO = 172,

    /* "name" and "pwd" to use when fetching. */
    USERNAME = 10173,
    PASSWORD = 10174,

        /* "name" and "pwd" to use with Proxy when fetching. */
    PROXYUSERNAME = 10175,
    PROXYPASSWORD = 10176,

    /* Comma separated list of hostnames defining no-proxy zones. These should
         match both hostnames directly, and hostnames within a domain. For
         example, local.com will match local.com and www.local.com, but NOT
         notlocal.com or www.notlocal.com. For compatibility with other
         implementations of this, .local.com will be considered to be the same as
         local.com. A single * is the only valid wildcard, and effectively
         disables the use of proxy. */
    NOPROXY = 10177,

    /* block size for TFTP transfers */
    TFTP_BLKSIZE = 178,

    /* Socks Service */
    SOCKS5_GSSAPI_SERVICE = 10179,

    /* Socks Service */
    SOCKS5_GSSAPI_NEC = 180,

    /* set the bitmask for the protocols that are allowed to be used for the
         transfer, which thus helps the app which takes URLs from users or other
         external inputs and want to restrict what protocol(s; to deal
         with. Defaults to CURLPROTO_ALL. */
    PROTOCOLS = 181,

    /* set the bitmask for the protocols that libcurl is allowed to follow to,
         as a subset of the CURLOPT_PROTOCOLS ones. That means the protocol needs
         to be set in both bitmasks to be allowed to get redirected to. Defaults
         to all protocols except FILE and SCP. */
    REDIR_PROTOCOLS = 182,

    /* set the SSH knownhost file name to use */
    SSH_KNOWNHOSTS = 10183,

    /* set the SSH host key callback, must point to a curl_sshkeycallback
         function */
    SSH_KEYFUNCTION = 20184,

    /* set the SSH host key callback custom pointer */
    SSH_KEYDATA = 10185,

    /* set the SMTP mail originator */
    MAIL_FROM = 10186,

    /* set the SMTP mail receiver(s; */
    MAIL_RCPT = 10187,

    /* FTP: send PRET before PASV */
    FTP_USE_PRET = 188,

    /* RTSP request method (OPTIONS, SETUP, PLAY, etc...; */
    RTSP_REQUEST = 189,

    /* The RTSP session identifier */
    RTSP_SESSION_ID = 10190,

    /* The RTSP stream URI */
    RTSP_STREAM_URI = 10191,

    /* The Transport: header to use in RTSP requests */
    RTSP_TRANSPORT = 10192,

    /* Manually initialize the client RTSP CSeq for this handle */
    RTSP_CLIENT_CSEQ = 193,

    /* Manually initialize the server RTSP CSeq for this handle */
    RTSP_SERVER_CSEQ = 194,

    /* The stream to pass to INTERLEAVEFUNCTION. */
    INTERLEAVEDATA = 10195,

    /* Let the application define a custom write method for RTP data */
    INTERLEAVEFUNCTION = 20196,

    /* Turn on wildcard matching */
    WILDCARDMATCH = 197,

    /* Directory matching callback called before downloading of an
         individual file (chunk; started */
    CHUNK_BGN_FUNCTION = 20198,

    /* Directory matching callback called after the file (chunk;
         was downloaded, or skipped */
    CHUNK_END_FUNCTION = 20199,

    /* Change match (fnmatch-like; callback for wildcard matching */
    FNMATCH_FUNCTION = 20200,

    /* Let the application define custom chunk data pointer */
    CHUNK_DATA = 10201,

    /* FNMATCH_FUNCTION user pointer */
    FNMATCH_DATA = 10202,

    /* send linked-list of name:port:address sets */
    RESOLVE = 10203,

    /* Set a username for authenticated TLS */
    TLSAUTH_USERNAME = 10204,

    /* Set a password for authenticated TLS */
    TLSAUTH_PASSWORD = 10205,

    /* Set authentication type for authenticated TLS */
    TLSAUTH_TYPE = 10206,

    /* Set to 1 to enable the "TE:" header in HTTP requests to ask for
         compressed transfer-encoded responses. Set to 0 to disable the use of TE:
         in outgoing requests. The current default is 0, but it might change in a
         future libcurl release.

         libcurl will ask for the compressed methods it knows of, and if that
         isn't any, it will not ask for transfer-encoding at all even if this
         option is set to 1.

    */
    TRANSFER_ENCODING = 207,

    /* Callback function for closing socket (instead of close(2;;. The callback
         should have type curl_closesocket_callback */
    CLOSESOCKETFUNCTION = 20208,
    CLOSESOCKETDATA = 10209,

    /* allow GSSAPI credential delegation */
    GSSAPI_DELEGATION = 210,

    /* Set the name servers to use for DNS resolution */
    DNS_SERVERS = 10211,

    /* Time-out accept operations (currently for FTP only; after this amount
         of miliseconds. */
    ACCEPTTIMEOUT_MS = 212,

    /* Set TCP keepalive */
    TCP_KEEPALIVE = 213,

    /* non-universal keepalive knobs (Linux, AIX, HP-UX, more; */
    TCP_KEEPIDLE = 214,
    TCP_KEEPINTVL = 215,

    /* Enable/disable specific SSL features with a bitmask, see CURLSSLOPT_* */
    SSL_OPTIONS = 216,

    /* Set the SMTP auth originator */
    MAIL_AUTH = 10217,

    /* Enable/disable SASL initial response */
    SASL_IR = 218,

    CURLOPT_LASTENTRY = 0 /* the last unused */
}

