package org.acme;

public class Response {
    private String msg = "OK";
    private int code = 0;
    private Object data = new Object();

    public String getMsg() {
        return msg;
    }

    public int getCode() {
        return code;
    }

    public Object getData() {
        return data;
    }
}