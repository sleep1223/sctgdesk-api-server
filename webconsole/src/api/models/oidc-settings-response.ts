/* tslint:disable */
/* eslint-disable */
/**
 * sctgdesk-api-server
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

import { Provider } from './provider';
 /**
 * 
 *
 * @export
 * @interface OidcSettingsResponse
 */
export interface OidcSettingsResponse {

    /**
     * @type {number}
     * @memberof OidcSettingsResponse
     */
    maxAuthCount: number;

    /**
     * @type {string}
     * @memberof OidcSettingsResponse
     */
    callbackUrl: string;

    /**
     * @type {Array<Provider>}
     * @memberof OidcSettingsResponse
     */
    providers: Array<Provider>;
}
