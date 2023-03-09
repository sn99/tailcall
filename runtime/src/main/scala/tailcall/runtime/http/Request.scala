package tailcall.runtime.http

import zio.http.model.Headers
import zio.http.{Body, Request => ZRequest, URL}
final case class Request(
  url: String = "",
  method: Method = Method.GET,
  headers: Map[String, String] = Map.empty,
  body: Array[Byte] = Array.empty
) {
  def toZHttpRequest: ZRequest =
    ZRequest(
      method = method.toZMethod,
      url = URL.fromString(url).getOrElse(throw new IllegalArgumentException(s"Invalid URL: $url")),
      headers = Headers(headers.map(header => Headers.Header(header._1, header._2))),
      version = zio.http.model.Version.`HTTP/1.1`,
      remoteAddress = None,
      body = Body.fromChunk(zio.Chunk.fromArray(body))
    )
}
