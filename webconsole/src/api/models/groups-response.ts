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

import { Group } from './group';
 /**
 * 
 *
 * @export
 * @interface GroupsResponse
 */
export interface GroupsResponse {

    /**
     * @type {string}
     * @memberof GroupsResponse
     */
    msg: string;

    /**
     * @type {number}
     * @memberof GroupsResponse
     */
    total: number;

    /**
     * @type {Array<Group>}
     * @memberof GroupsResponse
     */
    data: Array<Group>;
}